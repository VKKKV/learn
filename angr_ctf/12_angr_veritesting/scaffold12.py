# When you construct a simulation manager, you will want to enable Veritesting:
# project.factory.simgr(initial_state, veritesting=True)
# Hint: use one of the first few levels' solutions as a reference.

import sys

import angr


def main(argv):
    path_to_binary = argv[1]
    project = angr.Project(path_to_binary)
    initial_state = project.factory.entry_state(
        add_options={
            angr.options.SYMBOL_FILL_UNCONSTRAINED_MEMORY,
            angr.options.SYMBOL_FILL_UNCONSTRAINED_REGISTERS,
        }
    )

    project.hook(0x804fab0, angr.SIM_PROCEDURES['libc']['printf']())
    project.hook(0x804fb10, angr.SIM_PROCEDURES['libc']['scanf']())
    project.hook(0x80503f0, angr.SIM_PROCEDURES['libc']['puts']())
    project.hook(0x8048d60, angr.SIM_PROCEDURES['glibc']['__libc_start_main']())

    simulation = project.factory.simgr(initial_state, veritesting=True)

    def is_successful(state):
        stdout_output = state.posix.dumps(sys.stdout.fileno())
        return "Good Job.".encode() in stdout_output  # :boolean

    def should_abort(state):
        stdout_output = state.posix.dumps(sys.stdout.fileno())
        return "Try again.".encode() in stdout_output  # :boolean

    simulation.explore(find=is_successful, avoid=should_abort)

    if simulation.found:
        solution_state = simulation.found[0]
        print(solution_state.posix.dumps(sys.stdin.fileno()).decode())
    else:
        raise Exception("Could not find the solution")


if __name__ == "__main__":
    main(sys.argv)
