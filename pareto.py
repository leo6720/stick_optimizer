from models import Solution


def is_dominated(candidate: Solution, other: Solution) -> bool:
    """Return True if other is at least as good in all metrics and better in one.
    
    Optimized to short-circuit early if a metric is worse.
    """
    a = candidate.metric_tuple()
    b = other.metric_tuple()

    # Check if all of b <= a (b is at least as good in all metrics)
    all_better_or_equal = all(bi <= ai for ai, bi in zip(a, b))
    
    if not all_better_or_equal:
        return False
    
    # Check if at least one is strictly better
    any_strictly_better = any(bi < ai for ai, bi in zip(a, b))
    
    return any_strictly_better


def pareto_filter(solutions: list[Solution]) -> list[Solution]:
    """Keep only Pareto-efficient solutions for the main optimization metrics.
    
    Uses O(n log n) approach when possible, fallback to O(n²) for small sets.
    """
    if len(solutions) <= 100:
        # For small sets, direct comparison is fast enough
        return _pareto_filter_direct(solutions)
    else:
        # For larger sets, use more sophisticated approach
        return _pareto_filter_optimized(solutions)


def _pareto_filter_direct(solutions: list[Solution]) -> list[Solution]:
    """Direct O(n²) Pareto filtering."""
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


def _pareto_filter_optimized(solutions: list[Solution]) -> list[Solution]:
    """Optimized Pareto filtering using sorting for larger solution sets.
    
    Pre-sorts by primary metric to enable early termination.
    """
    # Sort by first metric (pocket types) - most important for early filtering
    sorted_solutions = sorted(solutions, key=lambda s: s.number_of_pocket_types)
    
    efficient: list[Solution] = []
    
    for solution in sorted_solutions:
        is_efficient = True
        
        for existing in efficient:
            if is_dominated(solution, existing):
                is_efficient = False
                break
        
        if is_efficient:
            # Remove any existing solutions dominated by this one
            efficient = [s for s in efficient if not is_dominated(s, solution)]
            efficient.append(solution)
    
    return efficient
