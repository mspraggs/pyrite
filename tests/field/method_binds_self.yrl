// foo1
// 0
class Foo {
  fn say_name(self) {
    print(self.name);
  }
}
var foo1 = Foo();
foo1.name = "foo1";
var foo2 = Foo();
foo2.name = "foo2";
foo2.fun = foo1.say_name;
foo2.fun();
