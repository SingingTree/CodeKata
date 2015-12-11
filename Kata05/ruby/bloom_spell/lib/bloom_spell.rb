require "bloom_spell/version"

class BloomSpeller
  def load_wordlist(file_name)
    File.readlines(file_name).each do
      puts 'foo'
    end
  end
end

bs = BloomSpeller.new
bs.load_wordlist('wordlist.txt')
