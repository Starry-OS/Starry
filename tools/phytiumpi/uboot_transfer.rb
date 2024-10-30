require 'serialport'

class UbootTransfer
  def initialize(device, baud, file_path)
    @device = device
    @baud = baud
    @file_path = file_path
  end

  def check_device
    unless File.exist?(@device)
      puts "Device #{@device} does not exist"
      exit(1)
    end
  end

  def transfer
    check_device

    puts "Open serial device"
    ser = SerialPort.new(@device, @baud, 8, 1, SerialPort::NONE)

    begin
      loop do
        line = ser.readline.strip
        puts line unless line.empty?
        break if line.include?('Hit any key')

        ser.write("\r\n")
      end

      loop do
        line = ser.readline.strip
        puts line unless line.empty?
        break if line.include?('Phytium-Pi#')

        ser.write("usb start; fatload usb 0 0x90100000 #{@file_path}\r\n")
        ser.write("go 0x90100000\r\n")
      end

      loop do
        user_input = gets.chomp
        ser.write("#{user_input}\r\n")
        line = ser.readline.strip
        puts line unless line.empty?
        break if user_input == 'exit'
      end
    rescue IOError => e
      puts "Serial error: #{e.message}"
    ensure
      ser.close
    end
  end
end

if __FILE__ == $PROGRAM_NAME
  puts "-- Uboot Transfer --"
  unless ARGV.length == 3
    puts "Usage: ruby uboot_transfer.rb <device> <baud> <file_path>"
    exit(1)
  end

  device = ARGV[0]
  baud = ARGV[1].to_i
  file_path = ARGV[2]

  uboot_transfer = UbootTransfer.new(device, baud, file_path)
  uboot_transfer.transfer
end
