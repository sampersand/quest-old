$DEBUG = true

require_relative 'quest'

l = Quest::List.new [1.to_q, "A".to_q, 3.to_q, 'b'.to_q, 91.3.to_q]

# puts l.call_attr(:index, 2.to_q, 3.to_q)
p 12.to_q.call_attr :<, 12.to_q
p 12.to_q.call_attr :<=, 12.to_q
p 12.to_q.call_attr :>, 12.to_q
p 12.to_q.call_attr :>=, 12.to_q
p 12.to_q.call_attr :!=, 12.to_q
p 12.to_q.call_attr :==, 12.to_q
# puts l.call_attr :index, 2.to_q

# puts l.call_attr :*, 2.to_q#[1.to_q,2.to_q].to_q
# p l.call_attr :*, [1.to_q, 2.to_q, 3.to_q].to_q
# puts l.call_attr(:@text).call_attr(:+, "A".to_q).call_attr(:@text_inspect).__text

# p [1,2,3] * [1,2,3]

# num = Quest::Number.new(1)
# text = Quest::Text.new('12345')
# puts text.call_attr(:[]=, 2.to_q, 3.to_q, '@'.to_q)
# # p text.call_attr(:each, Quest::Block.new{ |a| ::Kernel::puts a })
# Quest::Block.new{|a|
# 	::Kernel::puts "a: ".to_q.call_attr(:+, a);
# 	a
# }.call_attr(:'()', text)
