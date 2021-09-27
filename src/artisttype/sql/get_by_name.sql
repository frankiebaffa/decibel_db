select artype.*
from DecibelDb.ArtistType as artype
where artype.Name = :name
ane artype.Active = 1;
