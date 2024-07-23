import { Universe } from './pkg';

// init()

const universe = Universe.new(64, 64);
universe.render()

const renderLoop = () => {
    universe.tick()
    universe.render()
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);