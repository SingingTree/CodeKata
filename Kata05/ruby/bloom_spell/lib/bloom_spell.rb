require 'bloom_spell/version'
require 'bitarray'
require 'openssl'

class BloomSpeller
  def initialize
    @bit_array_size = 1_000_000
    @bit_array = BitArray.new(@bit_array_size)
    @digest = OpenSSL::Digest::SHA256.new
    @num_chunks = 4
  end

  def bloom_word(word)
    hash = @digest.hexdigest(word)
    chunk_size = hash.length / @num_chunks
    for i in 0..@num_chunks - 1
      sub_hash = hash[i..i + chunk_size - 1]
      @bit_array[sub_hash.to_i(16) % @bit_array_size] = 1
    end
  end

  def load_wordlist(file_name)
    File.readlines(file_name).each do |line|
      bloom_word(line)
    end
  end

  def check_word(word)
    hash = @digest.hexdigest(word)
    chunk_size = hash.length / @num_chunks
    for i in 0..@num_chunks - 1
      sub_hash = hash[i..i + chunk_size - 1]
      if @bit_array[sub_hash.to_i(16) % @bit_array_size] == 0
        return false
      end
    end
    true
  end
end

bs = BloomSpeller.new
bs.load_wordlist('wordlist.txt')

puts "cats: #{bs.check_word('cats')}"
puts "catz: #{bs.check_word('catz')}"
