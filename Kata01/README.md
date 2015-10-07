# Kata 01: Supermarket Pricing

A writeup by Bryce Guy.

## Does fractional money exist?

Almost certainly (and not even cents, fractions of those)! If you have a price specified as 1.99$ a pound, then if someone buys a kilogram of said item (it's probably generic brand bolts), then they're buying 2.205~ pounds. Or 4.38795$~ worth of bolts: fractional money.

Specials could also result in fractional amounts of money. If generic brand waffle mix costs 4.99 a carton normal, but it's international waffle day so it's 20% off waffle mix, then the resultant price would be 3.992.

There are more examples like this (buying bulk stock would also have it, me thinks), and these scenarios describe the calculation of a price, where I think it's reasonable to assume fractional currency. However, when it comes to paying, I don't think you can use fractional currency.

When people have to pay for their generic brand waffle and bolt mix, you can't charge them in fractional currency. You can screw them with a whole lot of tiny coin change, but you can't go smaller than cents. Even with an electronic system, I wouldn't imagine they provide such functionality. So in terms of the charge rendered to customers you don't have fractional money.

## When (if ever) does rounding take place?

As someone is paying for the item, and on that specific item. So when someone is putting the bolts through the checkout, rounding should occur on them at that point and they're charged a certain amount for the bolts specifically.

It would be possible to have all the of fractional costs summed before rounding, but then the fractional costs of each item would need to be presented on receipts and also how the rounding worked, which isn't conventional.
