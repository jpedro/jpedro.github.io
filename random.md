# Random Thoughts


## The most undersold value of the cloud

Their single API. That's it.

Why so important? Because AU. TO. MA. TION.

Those API calls are the smallest lego pieces. Atop that, you can
assemble walls, rooms, flats, houses, buildings. Whole cities in the
cloud can be captured in a git repo.


## Sometimes you don't need the cloud

Deploy that app straight into a bare host. Use unix sockets, avoid the
whole TCP/IP network overheads. And boom! It's the best available
performance you will _ever_ get.


## On a knife's edge

[In this comment](https://news.ycombinator.com/item?id=39366352)
someone remarked:

> all companies are operating on a knife's edge

And it makes sense, when you think about it.

It's the outcome of:

> if ain't broke don't fix it

Companies invest considerable time and money to get things off the
ground and make them work. I'd say even to **barely** make it work.

Because, drumrol... most projects are completed beyond schedule and
over budget.

As soon as the minimum acceptance is achieved there's zero or negative
incentives to improve it. To polish it. Refactor it. But also to
revisit the tech debt left behind.

Touching code that _barely_ works becomes a risky adventure, that only
the approved go, and only when there's a clear reward for it.

After the struggles to _barely_ make it work, developers are keen to
move to shinnier things. Managers want to close projects as "Mission
Accomplished" (_cue George W. Bush_) and end the apologising.

Over time, fresh meat assigned to add features or fix stuff are only
allowed to touch the minimal amount of code, in the smallest amount
of time. Say hello to cruft and the whole broken windows theory.

And every 10 years, it's time for that big-bang rewrite.
