require_relative 'quest'

num = Quest::Number.new(12)
p num
# text = Quest::Text.new('Abc')
# p text.call_attr(:each, Quest::Block.new{ |a| ::Kernel::puts a })