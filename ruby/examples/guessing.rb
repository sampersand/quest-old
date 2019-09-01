require_relative 'util'
class Quest::Pristine
	def call *a;
		call_attr :'()', *a
	end
	alias :g :get_attr
	alias :c :call
end

Quest::Object.get_attr(:init).call(_{
	max = 100._
	secret = g(:rand).c(1._, max)
	guesses = 0._
	guess = nil._
	g(:while).c(_{ guess.g(:!=).c(secret) }, _{
		guess = g(:prompt).c(
			"Guess from 1-"._.g(:+).c(max.g(:@text).c())
		).g(:@num).c();

		guesses.g(:'++@').c();

		g(:disp).c(
			g(:if).c(
				guess.g(:<).c(secret),
				_{ 'too low!'._ },
				_{
					g(:if).c(
						guess.g(:>).c(secret),
						'too high!'._,
						'perfect!'._
					)
				}
			).c()
		);
	});
	g(:disp).c("It took you "._.g(:+).c(guesses.g(:@text).c()).g(:+).c(" tries!"))
});
# secret = get_attr(:rand)