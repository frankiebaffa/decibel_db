select altype.*
from DecibelDb.AlbumTypes as altype
where altype.Name = :name
and altype.Active = 1;
