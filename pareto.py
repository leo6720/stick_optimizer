from models import Solution


def is_dominated(candidate: Solution, other: Solution) -> bool:
    """Return True if other is at least as good in all metrics and better in one."""
    a = candidate.metric_tuple()
    b = other.metric_tuple()

    return all(bi <= ai for ai, bi in zip(a, b)) and any(
        bi < ai for ai, bi in zip(a, b)
    )


def pareto_filter(solutions: list[Solution]) -> list[Solution]:
    """Keep only Pareto-efficient solutions for the main optimization metrics."""
    efficient: list[Solution] = []

    for i, solution in enumerate(solutions):
        dominated = False

        for j, other in enumerate(solutions):
            if i == j:
                continue

            if is_dominated(solution, other):
                dominated = True
                break

        if not dominated:
            efficient.append(solution)

    return efficient
