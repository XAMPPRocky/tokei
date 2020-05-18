// 26 lines 14 code 7 comments 5 blanks
namespace Ns
{
    /*

    multi-line comment

    */
    public class Cls
    {
        private const string BasePath = @"a:\";

        [Fact]
        public void MyTest()
        {
            // Arrange.
            Foo();

            // Act.
            Bar();

            // Assert.
            Baz();
        }
    }
}
