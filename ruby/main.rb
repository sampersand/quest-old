$DEBUG = true
require_relative 'quest'
require_relative 'examples/frac'
__END__
Frac = Quest::Object.call_attr(:birth)
Frac.set_attr :@num, Quest::Block::new {
	get_attr(:numer).call_attr :/, get_attr(:denom)
}
Frac.set_attr :birth, Quest::Block::new { |numer, denom| 
	frac = get_attr(:__parent__)
		.get_attr(:birth)
		.call_attr(:bind, self)
		.call_attr(:'()')
	frac.set_attr :numer, numer
	frac.set_attr :denom, denom
	::Kernel::p get_attr(:__parent__).get_attr(:birth).call_attr(:bind, self)
	frac
}

frac = Frac.call_attr(:birth, 48.to_q, 12.to_q)
p 12.to_q.call_attr :+, frac
# frac = Frac.call_attr(:birth)
# frac.set_attr :numer, 



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
