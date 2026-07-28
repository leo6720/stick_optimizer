import json
from dataclasses import asdict
from typing import Any
from models import GlobalSettings, Weights, StickType, Format, Candidate, Solution

def serialize_project(
    settings: GlobalSettings,
    weights: Weights,
    stick_types: list[StickType],
    formats: list[Format],
    results: list[Solution],
    active_filters: dict,
    selected_index: int | None
) -> str:
    """Serializes the entire project state to a JSON string."""
    
    def solution_to_dict(sol: Solution) -> dict:
        d = asdict(sol)
        # Convert sets to lists for JSON serialization
        d["pocket_types"] = sorted(list(d["pocket_types"]))
        d["robot_head_types"] = sorted(list(d["robot_head_types"]))
        return d

    project_data = {
        "settings": asdict(settings),
        "weights": asdict(weights),
        "stick_types": [asdict(s) for s in stick_types],
        "formats": [asdict(f) for f in formats],
        "results": [solution_to_dict(r) for r in results],
        "active_filters": active_filters,
        "selected_index": selected_index
    }
    return json.dumps(project_data, indent=4)

def deserialize_project(json_str: str) -> dict[str, Any]:
    """Deserializes project state from JSON string."""
    data = json.loads(json_str)
    
    data["settings"] = GlobalSettings(**data["settings"])
    data["weights"] = Weights(**data["weights"])
    data["stick_types"] = [StickType(**s) for s in data["stick_types"]]
    data["formats"] = [Format(**f) for f in data["formats"]]
    
    reconstructed_results = []
    for sol_dict in data["results"]:
        candidates = []
        for cand_dict in sol_dict.pop("candidates"):
            if cand_dict.get("pocket_type"):
                cand_dict["pocket_type"] = tuple(cand_dict["pocket_type"])
            if cand_dict.get("robot_head_type"):
                cand_dict["robot_head_type"] = tuple(cand_dict["robot_head_type"])
            candidates.append(Candidate(**cand_dict))
        
        # Reconstruct sets
        sol_dict["pocket_types"] = set(tuple(x) for x in sol_dict.get("pocket_types", []))
        sol_dict["robot_head_types"] = set(tuple(x) for x in sol_dict.get("robot_head_types", []))
        
        reconstructed_results.append(Solution(candidates=candidates, **sol_dict))
    
    data["results"] = reconstructed_results
    return data
