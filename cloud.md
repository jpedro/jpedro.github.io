<!-- hidden -->
# You might not need the cloud

This is not a click bait. Firstly because no one reads this and
secondly because some nuance will be prefaced.


## The benefits

Public cloud providers (AWS, Azure, GCP) are great when it comes to
scalability and, in many cases, ease of operations.

You can order a hundreds of VMs in minutes, if your limits, cloud and
credit card, allows that. This fact alone is impressive, compare to the
weeks and even months purchasing and configuring a bare metal machine
entails.

The cloud shines in an important and very under-appreciated aspect.

Automation.

Public cloud provide public APIs. This single entrypoint is where
ecosystems of SDKs, IaC and increasingly complex ways to call those
APIs are further built upon (_looking at crossplane_).

Also, public clouds make the hard parts trivial. Imagine configuring
firewalls across hundreds on bare metal machines with a single command.
Or creating read replicas with a single call. Or creating an S3-like
storage that's so flexible and feels infinite.


## Why most products don't need
