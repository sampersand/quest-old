require_relative '../object'

# The `Enumerable` "mixin"; it provides methods
# that anything that can be enumerated over (ie has the `each` method
# defined) can use.
class Quest::StepParents::Enumerable < Quest::Object
	# The only ancestor is Object.
	# We're only expecting the `each` method
	define_attrs ancestors: [ ::Quest::Object ] do

		define_attr :map do |meth|
			map = []
			call_attr(:each, ::Quest::Block.new{|obj|
				map.push meth.call_attr :'()', obj
			})
			::Quest::List.new map
		end

		define_attr :filter do |meth|
			map = []
			call_attr(:each, ::Quest::Block.new{|obj|
				map.push obj if meth.call_attr(:'()', obj).call_into :@bool
			})
			call_attr :init_map_result, ::Quest::List.new(map)
		end

		define_attr :init_map_result do |list|
			list
		end

		define_attr :find do |meth|
			call_attr(:try, ::Quest::Block.new{ |uniq_obj|
				call_attr(:each, ::Quest::Block.new{ |obj|
					if meth.call_attr(:'()', obj).call_into :@bool
						call_attr :return, uniq_obj, obj
					else
						::Quest::Null.new
					end
				})
				::Quest::Null.new
			})
		end

		define_attr :all do |meth|
			call_attr(:try, ::Quest::Block.new{ |uniq_obj|
				call_attr(:each, ::Quest::Block.new{ |obj|
					if meth.call_attr(:'()', obj).call_into(:@bool)
						::Quest::Null.new
					else
						call_attr :return, uniq_obj, ::Quest::Boolean.new(false)
					end
				})
				::Quest::Boolean.new true
			})
		end

		define_attr :any do |meth|
			call_attr(:try, ::Quest::Block.new{ |uniq_obj|
				call_attr(:each, ::Quest::Block.new{ |obj|
					if meth.call_attr(:'()', obj).call_into(:@bool)
						call_attr :return, uniq_obj, ::Quest::Boolean.new(true)
					else
						::Quest::Null.new
					end
				})
				::Quest::Boolean.new false
			})
		end

		define_attr :none do |meth|
			call_attr(:any, meth).call_attr :'!'
		end

		# define_attr :map_filter do |meth|
		# 	::Quest::List::new @list.map{|ele|
		# 		map = meth.call_attr :'()', ele
		# 		next map if map.call_into :@bool
		# 	}.compact
		# end


		# define_attr :find do |meth|
		# 	@list.find{ |ele| meth.call_attr(:'()', ele).call_into :@bool } || ::Quest::Null.new
		# end

		# define_attr :map_find do |meth|
		# 	@list.each do |ele|
		# 		map = meth.call_attr :'()', ele
		# 		break map if map.call_into :@bool
		# 	end || ::Quest::Null.new
		# end

	end
end