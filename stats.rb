#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "tempfile"

do_commit = true
do_change = true
do_csv = false
for arg in ARGV do
  if arg == "--nocommit" || arg == "-nc"
    do_commit = false
  elsif arg == "--nochange" || arg == "--nop"
    do_change = false
  elsif arg == "--help" || arg == "-h"
    puts "Usage: stats.rb [--nocommit] [--nochange]"
    exit
  elsif arg == "--csv"
    do_csv = true
  end
end

# This is a simple script to count the number of lines in all Rust files in the
# current repository. It is not meant to be super efficient, and is mostly
# cobbled together with the help of GitHub CoPilot.

# get all rs files and calculate line count
def get_rs_files
  Dir.glob("**/*.rs")
end

def calculate_file_line_count(file)
  line_count = 0
  char_count = 0
  File.open(file, "r") do |f|
    f.each_line do |line|
      line_count += 1
      char_count += line.length
    end
  end
  { line_count: line_count, char_count: char_count }
end

def generate_badge_link(total_count)
  "![lines of code](https://img.shields.io/badge/lines%20of%20rust-#{total_count}-informational)"
end

total_count = 0
total_char_count = 0
entries = []
get_rs_files.each do |file|
  next if file.include?("target")

  calculated_counts = calculate_file_line_count(file)
  intermediate_count = calculated_counts[:line_count]
  intermediate_char_count = calculated_counts[:char_count]
  total_count += intermediate_count
  total_char_count += intermediate_char_count
  entries << { file: file, line_count: intermediate_count, char_count: intermediate_char_count }
  puts "#{file} #{intermediate_count} (#{intermediate_char_count} chars)"
end

puts "Total: #{total_count} (#{total_char_count} chars)"

if do_csv
  csv_content = "file,line_count,char_count\n"
  entries.each do |entry|
    csv_content += "#{entry[:file]},#{entry[:line_count]},#{entry[:char_count]}\n"
  end

  # Create new file in stats directory
  file_name = "stats.csv"
  file_path = File.join(Dir.pwd, "stats", file_name)
  FileUtils.mkdir_p(File.dirname(file_path))
  File.write(file_path, csv_content)
end

unless do_change
  puts "Skipping change"
  exit 0
end

puts "updating the readme badge..."
temp = Tempfile.new "readme_temp.md"
File.open("README.md", "r") do |f|
  f.each_line do |line|
    if line.start_with?("![lines of code]")
      temp.puts(generate_badge_link(total_count))
    else
      temp.puts line
    end
  end
end
temp.close
FileUtils.mv(temp.path, "README.md")

unless do_commit
  puts "Not committing changes"
  exit 0
end

puts "checking git"
git_status = `git status --porcelain`
if git_status.length == 0
  puts "nothing to commit"
else
  puts "staging..."
  puts "git add ."
  puts `git add .`
  puts "committing..."
  puts "git commit -m stats.rb"
  puts `git commit -m stats.rb`
end
