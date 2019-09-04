require_relative 'object'

class Hash
	def to_q; Quest::Map.new self end
end

class Quest::Map < Quest::Object
	def initialize map
		::Quest::if_debug do
			unless map.is_a? ::Hash
				::Quest::warn "Map::initialize received a non-Hash arg '#{map.inspect}'"
			end

			map.each do |key, val|
				unless ::Quest::quest_object? key
					::Quest::warn "Map::initialize recieved a non-Object key '#{key.inspect}"
				end

				unless ::Quest::quest_object? val
					::Quest::warn "Map::initialize recieved a non-Object val '#{val.inspect}"
				end
			end
		end

		@map = map
		super()
	end

	def clone
		::Quest::Map.new @map.clone
	end

	def __map
		@map
	end

	def inspect
		"Map(#{@map.inspect})"
	end


	define_attrs ancestors: [
		::Quest::StepParents::Enumerable,
		::Quest::StepParents::Comparable,
		::Quest::StepParents::TruthyContainers,
		::Quest::Object
	] do

		define_attr :@text do
			::Quest::Text.new "{" + @map.map{|k, v|
				l.call_attr(:@text_inspect).call_into(:@text)
				 .call_attr(:+, v.call_attr(:@text_inspect).call_into(:@text))
			}.join(', ') + "}"
		end

		define_attr :<=> do |rhs|
			::Quest::Number.new (@map <=> rhs.call_into(:@map)) || ::Float::NAN
		end

		define_attr :@list do
			::Quest::List.new @map.to_a.map(::Quest::List.:new)
		end

		define_attr :@map do
			clone
		end

		define_attr :length do
			::Quest::Number.new @map.length
		end

		define_attr :== do |rhs|
			::Quest::Boolean.new rhs.is_a?(::Quest::Map) && @map == rhs.__map
		end

		define_attr :+ do |rhs|
			::Quest::Map.new @map.clone.update rhs.call_into :@map
		end

		define_attr :'+=' do |rhs|
			@map = call_attr(:+, rhs).__map
			self
		end

		define_attr :map do |meth|
			::Quest::Map.new @map.map{ |key, val| 
				result = meth.call_attr(:'()', key, val)
				[result.call_attr(:[], ::Quest::Number.new(1)), result.call_attr(:[], ::Quest::Number.new(2))]
			# call_attr :init_map_result, ::Quest::List.new(map)
			}.to_h
		end

		define_attr :[] do |key|
			@map[key] || ::Quest::Null.new
		end

		define_attr :[]= do |key, val|
			@map[key] = val
		end

		define_attr :each do |block|
			@map.each do |key, val|
				block.call_attr :'()', key, val
			end
			self
		end
	end
end