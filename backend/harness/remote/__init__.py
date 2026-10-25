"""Managed-agent (antigravity) sandbox runs for Theseus.

    from harness.remote import run_module_remote, run_parallel, RemoteSpec, open_pr_remote

See backend/harness/remote/README.md.
"""

from .run_remote import RemoteResult, RemoteSpec, run_module_remote, run_parallel, resume_remote, reverify_job
from .open_pr import open_pr_remote
from ._common import PreflightError, run_preflight

__all__ = [
    "RemoteResult",
    "RemoteSpec",
    "run_module_remote",
    "run_parallel",
    "resume_remote",
    "reverify_job",
    "open_pr_remote",
    "PreflightError",
    "run_preflight",
]
