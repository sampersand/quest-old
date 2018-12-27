Car = {
	init = {
		@self = Car.clone();
		@self.set(`maker`, @1);
		@self.set(`wheels`, @2 or 4);
		@self.`.=`(`@text`, @self.get(`@text`));
		@self
	};

	@text = {
		disp(@self);
	};

	drive = {
		disp("hi");
	};
	@locals
}();

Car.`.=`(`()`, Car.get(`init`));

car = Car("honda");

car.get(`drive`)()

disp(car.`@text`());