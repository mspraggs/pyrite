// foo
// bar
// bar
// 0
class Foo {
  fn method_on_foo(self) { print("foo"); }
  fn override(self) { print("foo"); }
}
class Bar < Foo {
  fn method_on_bar(self) { print("bar"); }
  fn override(self) { print("bar"); }
}
var bar = Bar();
bar.method_on_foo();
bar.method_on_bar();
bar.override();
