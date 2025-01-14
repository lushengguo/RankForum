# RankForum

A decentralized forum where everyone has their score in some topic/field

Decenteralized is easy to understand, which means community autonomy and 
without platform censorship

Score would authorize influence in that topic/field. Consider forums nowadays,
everyone is able to publish posts and comments. It's a good thing which means
everybody can enjoy the benefits of the internet. But it's also annoying that
so many meaningless posts mess up your application. And someone who is not
familiar with this field talks nonsense in the comment section. It's growing
harder to get useful content online, most of us are exhausted from spending
time on phones because it is hard to filter gold from stones.

So, everyone is able to create a topic/field, and everyone has an initial score
of zero. Everybody is able to post on the topic/field, and their post has two
attributes: Poster's current score and history score when they posted it.

And how do we get the score? It's quite simple, score changes from up-votes and
down-votes. Consider log{100}(score) as your level, and your score changes like
the pseudo code below:

if received_up_vote:
    score += min(100^poster_level * 10, 100^upvoter_level)
else received_down_vote:
    negative_score = min(100^poster_level * 10, 100^upvoter_level)
    total_score_from_this_post -= negative_score
    if level(total_score_from_this_post) > poster_level:
        ban_account(poster)
        return
    if poster_score < negative_score:
        score = 0

eg:
Alice is on level 3
An up-vote from a level 1 user would make her gain 1 score
An up-vote from a level 2 user would make her gain 1'00 scores
An up-vote from a level 3 user would make her gain 1'00'00 scores
An up-vote from a level 4 user would make her gain 10'00'00 scores
An up-vote from a level 5 user would make her gain 10'00'00 scores either

This guarantees that you get/lose more score from those who are at a really high
level. But that acceleration rate is not too much, and it doesn't limit you from
getting scores from low-level users. You have to contribute to the community,
even if the post only benefits the low-level users. A tremendous amount of votes
would send you into a relatively high level either.

And the most important thing is that a user could limit the comment permission
to some level. Users at any level would see this post because knowledge has no
boundary. This would block those who are not familiar with this topic/field.
You could get useful information from this filter feature. And also you can
filter the posts that you can see in this topic/field. Those who love this
topic/field would cherish their account.

Let us see some useful cases:
In the mathematics field, it's really easy to divide people into multiple
levels. Anyone who wants to talk about some advanced field and get useful
advice from all over the world must have a high-level account which means they
must contribute to the community. This action helps low-level users to
understand some really useful concepts and grow in the math field.

Another case:
A bunch of people loves some idol and makes a topic to talk about everything
about him. In a regular forum, their posts are public and malicious users would
talk nonsense in their posts. But here, it takes time to be noticed by malicious
users. And by then the founding members already have some scores, they can
easily up-vote some people like them to a high level. Malicious users have to
spend a large amount of resources to reach a high level. And good users' down-
votes would send them back to the level without permission.