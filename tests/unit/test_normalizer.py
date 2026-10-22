"""test_normalizer package."""

import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "../../api/sil_api"))


def test_normalize_f_hook():
    """Test normalize f hook."""
    from core_bridge import run_sil_pipeline
    result = run_sil_pipeline("\u0192dev")
    assert result["decision"] == "REWRITE"
    assert result["normalized"] == "fdev"


def test_normalize_clean():
    """Test normalize clean."""
    from core_bridge import run_sil_pipeline
    result = run_sil_pipeline("hello")
    assert result["decision"] == "ALLOW"


def test_normalize_cyrillic_spoof():
    """Test normalize cyrillic spoof."""
    from core_bridge import run_sil_pipeline
    result = run_sil_pipeline("\u0440\u0430ypal")
    assert result["decision"] == "BLOCK"
