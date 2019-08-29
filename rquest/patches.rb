Object.define_method(:to_b){ true }
NilClass.define_method(:to_b){ false }
FalseClass.define_method(:to_b){ false }
Numeric.define_method(:to_b){ !zero? }
String.define_method(:to_b){ !empty? }
Array.define_method(:to_b){ !empty? }
Hash.define_method(:to_b){ !empty? }

class Object
	def __ATSIGN__num; proc{ respond_to?(:to_f) ? (f = to_f; f.floor == f.ceil ? to_i : f) : to_i } end
	def __ATSIGN__text; proc{ to_s } end
	def __ATSIGN__bool; proc{ to_b } end
	def __ATSIGN__list; proc{ to_a } end
	def __ATSIGN__map; proc{ to_h } end
	# p methods
	# exit
end

class NilClass
	def call *_, **__; nil end
end

class String
	def call
		Kernel::` self #` # sublime doesn't highlight this syntax correctly
	end
end

class Numeric
	def to_str; to_s end
end

class Integer
	alias :__old_plus :+
	def + rhs
		rhs.is_a?(String) ? to_s + rhs : __old_plus(rhs)
	end
end