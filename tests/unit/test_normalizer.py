import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "../../api/sil_api"))


def test_normalize_f_hook():
    from core_bridge import run_sil_pipeline
    result = run_sil_pipeline("ƒdev")
    assert result["decision"] == "REWRITE"
    assert result["normalized"] == "fdev"


def test_normalize_clean():
    from core_bridge import run_sil_pipeline
    result = run_sil_pipeline("hello")
    assert result["decision"] == "ALLOW"


def test_normalize_cyrillic_spoof():
    from core_bridge import run_sil_pipeline
    result = run_sil_pipeline("раypal")
    assert result["decision"] == "BLOCK"
