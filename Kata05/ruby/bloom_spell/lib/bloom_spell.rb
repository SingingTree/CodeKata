require 'bloom_spell/version'
require 'bitarray'
require 'openssl'

class BloomSpeller
  def initialize
    @bit_array_size = 100_000
    @bit_array = BitArray.new(@bit_array_size)
    @digest = OpenSSL::Digest::SHA256.new
  end

  def load_wordlist(file_name)
    File.readlines(file_name).each do |line|
      puts @digest.hexdigest(line)
    end
  end
end

bs = BloomSpeller.new
bs.load_wordlist('wordlist.txt')
