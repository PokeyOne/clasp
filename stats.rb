# frozen_string_literal: true

# This is a simple script to count the number of lines in all Rust files in the
# current repository. It is not meant to be super efficient, and is mostly
# cobbled together with the help of GitHub CoPilot.

# get all rs files and calculate line count
def get_rs_files
  Dir.glob("**/*.rs")
end

def calculate_file_line_count(file)
  count = 0
  File.open(file, "r") do |f|
    f.each_line do |line|
      count += 1
    end
  end
  count
end

total_count = 0
get_rs_files.each do |file|
  intermediate_count = calculate_file_line_count(file)
  total_count += intermediate_count
  puts "#{file} #{intermediate_count}"
end

puts "Total: #{total_count}"