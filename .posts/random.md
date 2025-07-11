# Random

[<img src="https://img.shields.io/badge/Mildly-Interesting-brightgreen">](/tags/mildly-interesting)


## Interviewing

Just read [Career Retrospective: Being Interviewed](https://thecodist.com/career-retrospective-being-interviewed/).

Andrew ends by saying:

> I never asked anyone to write code in an interview

I disagree with this statement. The post reflects on his experiences
being interviewed and you can see the progression from interviews being
human conversations to harder and more technical.

I insist on getting people to code on interviews. Real code, not
leetcode, on their preferred IDE or code editor. Normally, the smallest
example, that can be expanded or complicated as allowed by the
candidate.

I know this adds extra stress to the process but coding live helps
us to gain invaluable insight. We don't care if they complete the coding
challenge or not. We care how they search and filter information,
how they tweak code they find and think.


## The end

Sometimes a means to an end is not the mean to that end. Sometimes,
the "mean" is the **end goal itself**.


## Mastery

The internets tell me mastery is spending 10_000 hours doing
something. Cute but no. I prefer this:

> It's when you made ALL the possible mistakes you can. Over
> and over again. Even then, maybe...


## In my shoe

From my limited viewpoint, the worst managers are the ones that were
very technical at some point. And it's worse the more recently that
transition happened.

I'm not bragging but I know I would be a terrible manager. Because I
fuss about code. A comma or a space out of place done by someone else
is a like a particle in my shoe. I might not remove it immediately but
my brain knows it's theeeeere!

When you code, you have absolute control over every character you type.
That translate poorly when managing people.

[Peter Principle discussion on Hacker News](https://news.ycombinator.com/item?id=39844104)

[How that name started](https://www.youtube.com/watch?v=39wzku9KIEM)


## Sounds

Sometimes I wonder how much music or how many songs connect to us at a
deeper level because they resemble sounds we hear in the womb. Can we
see, smell, touch, taste in there? No, no, and probably not.

The sole source of stimuli are sounds.

Just think. In total darkness, floating in that warm, sensory-deprived
amniotic universe. It has be the ultimate ASMR experience.


## Undersold

The most undersold value of the cloud is their unified API. Why?

Because AU. TO. MA. TION!

Those API calls are the smallest lego pieces. The bricks. Atop
that, you can assemble walls, rooms, flats, houses, buildings. Whole
cities in the cloud can be captured in a repo, automated, packaged
as a library, some tool, service or platform.

Most ad-hoc scripts and home brewed provisioning tools can't even
agree on a standard for the sand that builds the bricks.

Totally unrelated: [I have a bridge to sell to you](/hosting).


## No, you don't!

Sometimes you don't need a "cloud". Deploy that app straight into
a bare host. Use unix sockets. Avoid TCP/IP overheads.

You better pray the backups and failover are working when things
fail (which they will) but until then, weeee! It's the best
performance you will _ever_ get.


## Progression

It's funny how things organically evolve in dev and ops.

You start by installing some tool and testing it with a few CLI commands.

When you use it enough you throw them into a bash script in the `$PATH`.
Extract those CLI flags into script arguments. Tidy it up, wrap it with a
few functions, subcommands, a useful usage help text, validate all inputs.

After that company hacktahon, now you got the greenlight to integrate that
into the app, like say creating image thumbnails. And sure enough, you can
shell these out. Or the tool has or the community created a SDK. Great.

Down the road, you realise that it's better to do this off the main thread
and do it asynchronously in the background. You extract that into its own
service. Throw the payload to some queue with a callback. Maybe deploy
this to some evented loop runtime. It's **a service** now. Maybe not a
Chad service but one nonetheless.

After a while you need to add some authentication because
[you don't want this to be public, right](https://x.com/ozgrozer/status/1838895852259041362)?
[Right](https://x.com/kaepora/status/1838651348797063276)?!
Because moved this out of some private subnets and split its own dev/test
environments. Anyway, there's now a pizza team of 2 only doing this now.
With separate monitoring, alerts channels. It's now **an app**.

Now with an auth token away (hopefully an OAuth or OIDC one) and
your main business tanking, you realise you can sell this cheaper than
crazy competition prices. So you do _that_. The app now needs its own
login endpoint. It's now **a product**.

Later you add enterprise features. Which in my book comes down to granular
access controls and integration with other services and platforms, like
customer SSO, notifications, maybe a marketplace, that allows 3rd parties to
hook up with ge public API.  Voilá. It's now **a platform**.

And there it is. The whole "journey":

Shell commands --> Script --> Function --> Module --> Library --> Service --> Product --> Platform.

