## Usage guide

To use this package, the recommended setup looks like this:
```js
const tickSpeed = 100;

const universe = Universe.new(64, 64);
universe.render()

const renderLoop = () => {
    universe.tick()
    universe.render()
    setTimeout(renderLoop, tickSpeed)
}

setTimeout(renderLoop, tickSpeed)
```

Alternatively, the `setTimeout` calls can be replaced by `requestAnimationFrame(renderLoop);` if you want the maximum tick speed and performance.

Don't forget to import `Universe` from the package!

**For a detailed description of the project, check out my GitHub Repo on https://github.com/tasamo23/rusty_life👍**