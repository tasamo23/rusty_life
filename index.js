import { Universe } from './pkg';

const universe = Universe.new(64, 64);
universe.render()

const renderLoop = () => {
    universe.tick()
    universe.render()
    setTimeout(renderLoop, 500)
    // requestAnimationFrame(renderLoop);
}

setTimeout(renderLoop, 100)
// requestAnimationFrame(renderLoop);