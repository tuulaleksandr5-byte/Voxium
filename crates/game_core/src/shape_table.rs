//! Таблица форм блоков частей 7–8 (создаётся скриптом регистрации).
use crate::{BlockId, models::Shape};

#[must_use]
pub(crate) fn lookup(id: BlockId) -> Option<Shape> {
    match id {
        1506..=1506 => Some(Shape::Campfire),
        1533..=1534 | 1667..=1667 => Some(Shape::Plate),
        1535..=1536 => Some(Shape::Fence),
        1537..=1546 => Some(Shape::Trapdoor),
        1547..=1562 => Some(Shape::Gate),
        1563..=1570
        | 1645..=1648
        | 1656..=1659
        | 1662..=1665
        | 1731..=1746
        | 1759..=1774
        | 1795..=1798
        | 1802..=1805
        | 1809..=1812
        | 1816..=1819 => Some(Shape::Stairs),
        1571..=1578 | 1668..=1671 => Some(Shape::Button),
        1579..=1610 => Some(Shape::Door),
        1611..=1618 => Some(Shape::Sign),
        1619..=1626 => Some(Shape::WallSign),
        1629..=1629 => Some(Shape::Composter),
        1639..=1642 | 1830..=1831 => Some(Shape::Potted),
        1649..=1649
        | 1660..=1660
        | 1672..=1672
        | 1800..=1800
        | 1807..=1807
        | 1814..=1814
        | 1821..=1821 => Some(Shape::Wall),
        1676..=1692 => Some(Shape::Candle),
        1693..=1709 => Some(Shape::CandleCake),
        1717..=1718 => Some(Shape::SculkSensor),
        1779..=1779 => Some(Shape::Rod),
        1787..=1787 => Some(Shape::Carpet),
        1789..=1789 => Some(Shape::Dripleaf),
        1837..=1837 => Some(Shape::Jar),
        _ => None,
    }
}

#[must_use]
pub(crate) fn is_slab(id: BlockId) -> bool {
    matches!(id, 1531..=1532 | 1650..=1650 | 1655..=1655 | 1666..=1666 | 1747..=1750 | 1775..=1778 | 1799..=1799 | 1806..=1806 | 1813..=1813 | 1820..=1820)
}

#[must_use]
pub(crate) fn is_cross(id: BlockId) -> bool {
    matches!(id, 1507..=1507 | 1513..=1513 | 1515..=1516 | 1522..=1522 | 1524..=1528 | 1711..=1714 | 1720..=1720 | 1780..=1780 | 1782..=1786 | 1790..=1792)
}

#[must_use]
pub(crate) fn is_flat(id: BlockId) -> bool {
    matches!(id, 1788..=1788 | 1835..=1835)
}
