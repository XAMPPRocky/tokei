// 26 lines 14 code 9 comments 3 blanks
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
