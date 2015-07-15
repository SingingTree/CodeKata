require 'spec_helper'

describe Chop do
  it 'has a version number' do
    expect(Chop::VERSION).not_to be nil
  end

  it 'chops an empty array' do
    expect(Chop::chop(3, [])).to eq(-1)
  end

  it "chops a single element array" do
    expect(Chop::chop(3, [1])).to eq(-1)
    expect(Chop::chop(1, [1])).to eq(0)
  end

  it "chops a three element array" do
    expect(Chop::chop(1, [1, 3, 5])).to eq(0)
    expect(Chop::chop(3, [1, 3, 5])).to eq(1)
    expect(Chop::chop(5, [1, 3, 5])).to eq(2)
    expect(Chop::chop(0, [1, 3, 5])).to eq(-1)
    expect(Chop::chop(2, [1, 3, 5])).to eq(-1)
    expect(Chop::chop(4, [1, 3, 5])).to eq(-1)
    expect(Chop::chop(6, [1, 3, 5])).to eq(-1)
  end

  it "chops a four element array" do
    expect(Chop::chop(1, [1, 3, 5, 7])).to eq(0)
    expect(Chop::chop(3, [1, 3, 5, 7])).to eq(1)
    expect(Chop::chop(5, [1, 3, 5, 7])).to eq(2)
    expect(Chop::chop(7, [1, 3, 5, 7])).to eq(3)
    expect(Chop::chop(0, [1, 3, 5, 7])).to eq(-1)
    expect(Chop::chop(2, [1, 3, 5, 7])).to eq(-1)
    expect(Chop::chop(4, [1, 3, 5, 7])).to eq(-1)
    expect(Chop::chop(6, [1, 3, 5, 7])).to eq(-1)
    expect(Chop::chop(8, [1, 3, 5, 7])).to eq(-1)
  end
end
