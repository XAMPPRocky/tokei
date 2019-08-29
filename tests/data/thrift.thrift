// 38 lines 29 code 2 comments 7 blanks

namespace java test
namespace py test

/* /* */
service Twitter extends core.BaseService {
    void ping(),

    bool postTweet(1: Tweet tweet) throws (1: TwitterUnavailable unavailable),

    TweetSearchResult searchTweets(1: string query),
}

enum TweetType {
    TWEET,       # 1 /*
    RETWEET = 2, // 2
    DM = 0xa,    // 3 */
    REPLY
}

struct Tweet {
    1: required i32 userId,
    2: required string userName = "/*",
    3: required string text = '...',
    4: optional Location loc,
    5: optional TweetType tweetType = TweetType.TWEET,
    16: optional string language = "en\"glish", // */
}

const string TEST1 = // "
    "starts here,
        test/*
        test" // a quote: "
const string TEST2 = /* " */
    'starts here,
        test,*/
        test' # another quote: "
