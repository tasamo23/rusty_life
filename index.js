import { Universe } from './pkg/rusty_life';

const tickSpeed = 100;

const universe = Universe.new(64, 64);
universe.render()

const renderLoop = () => {
    universe.tick()
    universe.render()
    setTimeout(renderLoop, tickSpeed)
}

setTimeout(renderLoop, tickSpeed)