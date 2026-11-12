# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_object-v1.0.xsd (OffsetObjectGroup, PointOnPathGroup)
# Changes:    OffsetObjectGroup (TransportLinkRef, LinkDistance) and the NetworkLinkRef / NetworkPathRef choice of PointOnPathGroup restored as flat optional fields on a device definition; distance in metres as a finite float; the XSD's "either a link or a path" choice is a Python-only rule
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-05
# Version:    1.0
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Where a device sits on the transport and network model.

A UTMC device definition could be tied to the transport link it monitors
(``TransportLinkRef``) and placed along it (``LinkDistance``, in metres), and
could name the network link or path it stands on (``NetworkLinkRef`` or
``NetworkPathRef``, one of the two). Without them a detector or sign has a
point on a map but no relationship to the link, path or route it serves.

The fields are declared on each definition that carries them, so their position
in the schema stays under that definition's control; the types and the choice
rule are shared from here.
"""

from typing import Annotated

from pydantic import Field

from topas_3000_model.common.ids import ObjectId

TransportLinkRef = Annotated[
    ObjectId,
    Field(description="Transport link this device monitors or sits on (the id of a transport link)."),
]

LinkDistanceMetres = Annotated[
    float,
    Field(allow_inf_nan=False, description="Distance of the device along its transport link, in metres."),
]

NetworkLinkRef = Annotated[
    ObjectId,
    Field(description="Network link the device stands on. At most one of this and the path may be given."),
]

NetworkPathRef = Annotated[
    ObjectId,
    Field(description="Network path the device stands on. At most one of this and the link may be given."),
]


def check_network_location(network_link_id: str | None, network_path_id: str | None) -> None:
    """The XSD allows a network link *or* a network path, never both."""
    if network_link_id is not None and network_path_id is not None:
        raise ValueError("give either networkLinkId or networkPathId, not both")
