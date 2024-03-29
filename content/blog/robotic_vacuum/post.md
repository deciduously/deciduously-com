---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--5juB8G4w--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/1fnrjap1v90ngs0q9axx.jpg
date: 2019-07-21T12:00:00.000Z
tags:
  - offtopic
  - help
title: I Am Mesmerized By Our New Robotic Vacuum
---

Disclaimer: this is more a rant than anything else. Maybe more #crazydudewontstoptalkingavoidtheofftopic but that's unwieldy as a hashtag. The #help part is in the TL;DR.

My girlfriend (somewhat impulsively) recently bought us a new robotic vacuum. We'd had the idea in the back of our heads for a while but I always assumed we were talking about at least a $500+ purchase having only heard of the big name brands like Roomba. Thanks to Lord Bezos, we got a [Eufy RoboVac 30](https://www.eufylife.com/products/variant/robovac-30/T2116111) for under $200 - selling at $270 from the manufacturer, still considerably cheaper than I had anticipated. It's a pretty well-reviewed "budget" model. I can't even bring myself to be that upset, this thing is really cool. Dammit, Amazon, it's so hard to quit you. Her only other purchase? God of War for PS4. I'm gonna keep her.

I swear I'm not a Eufy shill, I'd never even heard of this brand, I'm just geeking out over our new robot. I'm not necessarily specifically recommending this model over other similar-tier models.

We put it through its inaugural run this weekend and I am utterly fascinated by it. That little thing is pretty clever, more so than I had anticipated, and I've just pretty much wasted my morning following it around trying to get inside its software head. I know these things have been around a long time now, but I'd never really been around one long enough to see it do its thing, and I'm impressed at how thorough a job it's doing.

It seems to be a great example of emergent behavior from very simple parts, and I can't stop thinking about it.

I admit, I was initially a total elitist. Some cursory searching had led me to believe that the super fancy high end ones have this whole mapping feature that figures out our floor map and stores it to efficiently cover the area, using its knowledge of the layout to optimize the path whereas the cheaper ones bumble around with sensors to get the same job done but much less efficiently, and likely only to 100% coverage over multiple runs. Not so much a replacement for normal manual vacuuming but an extra help that's pretty good for day-to-day maintenance and touch-ups.

I hadn't really ever thought about the problem before, so when we opened it up I had initially thought the basic MO would be to just run in straight lines until its bumper hits something, then try a new angle. Rinse, repeat for X amount of time, floor will be eventually cleaned. Do it every day or multiple times a week, and over the course of a week your whole floor is clean. Our apartment is a little funky layout-wise, and with a decent amount of floor space but odd partitions and angles in places, so I figured without some sort of stored picture of the shape of the space there was no way it could reasonably cover the area in a single 100-minute shot-in-the-dark style run.

I owe the entire industry an apology for what's clearly been a gross underestimation. I know I should have known better, there's all these different companies making all these different models. It's the future now, and people have put some _thought_ into this even (especially?) at this price point.

This thing schooled _my_ ass on how to get the job done. I'm not even ashamed to say this is likely the cleanest these floors have been since we moved in a year ago. It's nuts that it got the job done just through an effective application of very simple movement algorithms. If it's not 100%, it's pretty damn close. I ended up following it around or watching it from my desk for the duration (which felt a little embarrassing, yes, but I couldn't stop looking) and don't believe a spot went untouched even if it got there in a pretty roundabout way. It even crawled up on an area mat without complaining and handled all raised thresholds between rooms like a champ.

It likely simply has better suction than our manual vacuum to boot, but that particular unit of measure (1500Pα) doesn't mean a lot to me. I couldn't tell you what a Pascal of suction feels like, and don't know what the other vacuum is rated. I do know it did a seriously good job considering it's so small.

While I don't have an uber-fancy model to compare to in the same space, this thing is leading me to believe the next level up probably wouldn't have even been worth it, at least for us. I suppose if we had a mansion and the battery life simply wasn't long enough for all of the floors the time saved by optimizing for our specific home might be useful. Watching it putter about, though, I actually almost believed it _was_ mapping the space out.

Instead, the manual and some searching around suggests it simply cycles through a few modes, based on data from its sensors. It's all very vague, but I think that might mean it really is quite simple. It starts by shooting off in a random direction, as expected, and turning when it detects an obstacle. It also has an edge-tracking mode with methodically tracks an edge by continuously attempting to turn right at a low speed, and a spot cleaning mode that spirals out from a point. You can enable each mode specifically or just let it auto-cycle, and I see no reason not to do that.

I had also underestimated the number of sensors - it's not just a big front bumper like I thought. It's got an infrared sensor on the side and a sensor on the bottom for drops, as well as a magnetic sensor to detect strips you can lay out to prevent it from going in certain areas. The infrared sensor may be just for the remote, but I think it might be used for locating home base as well if it's not storing any locations.

What got me to keep watching is how this cycling of simple algorithms managed to still cover _all_ previously uncovered territory. As I was watching it I kept thinking "oh no, it doesn't know that it just missed a whole corner over here", watching it scoot off at some random diagonal. I actually had assumed it would be a little more methodical, tracing a zigzag across a space, but no, this thing was all random unrelated directions, which kept it interesting enough (apparently). Inevitably ten or fifteen minutes later some other seemingly random tack across the house would bring it exactly where it needed to go. It triggered edge mode at least once on every wall of every room and every weird nook and corner, got under all chairs and the kitchen table, found the bathroom a few times and got wherever it could fit, just really did a smashing job of it all. We never had to intervene.

These new vectors its choosing can't be random. It works too well. They sure seem like they must be informed by some sort of previous understanding of found edges, but one guess I had is that it actually just tries a range of vectors over time to ensure it hits a wide range. My gut feeling is that it's actually tuned even better than that but I couldn't tell from watching the first run, and I'd be curious to learn more about that process. I also don't know if that behavior is specific to this model, and others do take a more zig-zag approach. Ideally I'd try it out, but I'm not ready to bring a second vacuuming robot into our lives. I'm not _that_ excessive, I swear.

As far as I can tell it's not storing where specifically it finds edges, and I did notice it repeat some work - for example, it methodically scooted along the same wall of the living room three times over the course of it's 100-minute run. If it were storing specific edges, it wouldn't have done so, so it must have just sensed an edge and kicked in to that mode.

It must just be calibrated properly to avoid the problem. For instance, it must know how often to kick in to edge mode to maximize for exploration and minimize repeated work without sacrificing thoroughness. 100 minutes is a long time to vacuum, but not that long if you waste 20 of it on one wall over and over again. Most of the time it just turns away, though, and sometimes it even anticipates a wall and turns before it hits it well before (I think) any of its sensors would have noticed it. I can't figure that out either if it's not keeping track of anything. It still manages to activate for all the different walls, though!

Due to a connecting door we keep closed for airflow purposes, in the robot's universe our unit is kind of U shaped. We have furniture in odd places and some large open areas and some small, more fractured non-rectangular areas. All six of the rooms we want it to cover are different shapes and sizes, and all have more than four walls due to nooks and crannies and closets of varying sizes. Most of the doorways are your standard door size, but the living room and dining room are connected by a big opening almost as wide as the wall but not quite. This creates even more weird edges and corners to confuse it. Optimizing a filing path in a given time constraint by hand even with prior knowledge would probably not be simple.

Somehow, amazingly, it just kept passing through rooms at different enough angles to cover the whole floor. Sometimes it'd be stuck in a room for a while, sometimes it would get on a tack that has it passing between two rooms regularly, back and forth in a big line, but by the time it hit low power mode and returned itself to the charger it had actually hit all the spots I had made mental notes to look out for.

It even surprised me once again when I got worried it ran out of battery too far from home. I heard it kick into low power mode, and it was in a completely different room facing the wrong way. It did know how to orient itself towards home and try to get there, but on the way there it hit the ottoman (so, it didn't anticipate that was coming) and turned around the wrong way again. It even still with no line of sight managed to get itself oriented towards the station, bump its way around the ottoman to a straight shot, and amble its way back onto the contact pins. Even better, this straight shot included a bit of edge mode - which I was pleased to see still enabled even in low power mode - along the front edge of the couch. I'd been taking note of unexplored edges and that was the only one remaining it hadn't done!

Okay. Enough gushing. Here's the point.

## TL;DR

I'm fascinated by how effective this budget-end robot vacuum's seemingly simple set of algorithms is at cleaning our whole weird space. I want to try to model the problem and implement the movement algorithms myself to see if I can replicate that emergent behavior with totally random inputs or if it requires more tuning, and what sort of tuning if so. It's not a type of programming I have much experience with. What would you use to explore this sort of thing?

I have some general space-filling/path-finding algorithms in my toolkit to start from, but I also know that some environments are easier than others for this sort of modeling. Some options I've heard of but don't know anything about:

- [Processing](https://processing.org/) - Java (I don't really know Java, but am studying C++? Similar-ish?)
- The [python turtle](https://docs.python.org/3.3/library/turtle.html?highlight=turtle) - Python. I don't know anything about turtle and very little about Python but it sounds like it can be used to explore this problem space. Python seems like a good choice for this but I don't know the ecosystem at all.
- I guess the [HTML canvas element](https://www.w3schools.com/html/html5_canvas.asp) but that sounds unwieldy and complicated. Is there a framework you recommend?
- I know Rust has some geospatial crates and image/graph crates for visualization, but nothing that I know of integrated for ease of use, experimentation and one-offs.
- Game engines? I've never used any, is that the right genre of tool for this? [Love2d](https://love2d.org/) for LUA looks like it might be a good choice but it also might be overkill.

...that's it, really. Is there anything else I should be aware of?

Also, if you know more about these robot vacuums than I do and can enlighten me/us, please do!

Photo by Rock'n Roll Monkey on Unsplash
