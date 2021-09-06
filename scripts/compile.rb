# frozen_string_literal: true

def project_root
  File.expand_path(File.join(__dir__, ".."))
end

def source_directory
  File.join(project_root, "src")
end

def common_directory
  File.join(source_directory, "common")
end

def compiler_directory
  File.join(source_directory, "compiler")
end

def runner_directory
  File.join(source_directory, "runtime_environment")
end

def common_files
  source_files = Dir.glob(File.join(common_directory, "**", "*.c"))
  source_files << Dir.glob(File.join(common_directory, "**", "*.h"))
  source_files.flatten!
end

def compiler_source_files
  source_files = common_files
  source_files << Dir.glob(File.join(compiler_directory, "**", "*.c"))
  source_files << Dir.glob(File.join(compiler_directory, "**", "*.h"))
  source_files.flatten!
end

def runner_source_files
  source_files = common_files
  source_files << Dir.glob(File.join(runner_directory, "**", "*.c"))
  source_files << Dir.glob(File.join(runner_directory, "**", "*.h"))
  source_files.flatten!
end

def compile_files(files:, name: "build_output")
  unless files
    puts "No files to compile"
    return
  end

  command = sprintf("gcc %s && mkdir -p ./build && mv ./a.out ./build/%s",
                    files.join(" "),
                    name)
  puts "running the command: \e[31m#{command}\e[0m"
  result = `#{command}`
  puts "and got back: #{result}"
end

def compile_compiler
  compile_files(files: compiler_source_files)
end

def compile_runner
  compile_files(files: runner_source_files)
end

def show_help
  puts "Invalid command"
end

##
# Main Program
##

if ARGV.length.zero?
  puts "Please specify a compile type"
  return
end

if ARGV[0] == "compiler"
  puts "Compiling the compiler..."
  compile_compiler
  puts "Done"
elsif ARGV[0] == "runner"
  puts "Compiling the runner"
  compile_runner
  puts "Done"
else
  show_help
end
