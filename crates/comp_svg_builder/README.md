
## Future Refactoring
Currently, each Entity represented by an SVG owns its own SVGBundle (a small SVG tree visualizing the Entity). Changes are applied at the end, which is a good concept for the ECS system 
but right now is poorly implemented and restrictive.
We need to simplify this to follow the KISS (Keep It Simple Stupid) principle.

1. **Initialization**: Create the SVGBundle in the first step (Hierarchy doesn't matter here).
2. **Update**: Update the SVGBundle, by applying changes from Components. 
An SVGElement can have two types of children: an Entity (WorldContext) or another SVGElement. Given the small size of each subtree, finding the correct SVGElement to update should be quick.
3. **Hierarchy Update**: During the update event, establish the hierarchy by checking which Entity is a child of another and update the SVGElement children.
4. **Frontend Update (if applicable)**: Check which SVGBundles have changed, verify the HierarchyIndex and child index, and send the events in the correct order to the frontend.

As SVGBundle sub tree we could use [RCTree](https://github.com/RazrFalcon/rctree/blob/master/src/lib.rs) and use [SVGDOM](https://github.com/RazrFalcon/svgdom/tree/master) as inspiration,
this should make things more generic and flexible?
