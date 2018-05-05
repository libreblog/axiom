#pragma once

#include "../grid/GridSurface.h"
#include "../ModelObject.h"

namespace AxiomModel {

    class Control;

    class ControlSurface : public ModelObject {
    public:
        using ChildCollection = CollectionView<Control*>;

        ControlSurface(const QUuid &uuid, const QUuid &parentUuid, AxiomModel::ModelRoot *root);

        static std::unique_ptr<ControlSurface> deserialize(QDataStream &stream, const QUuid &uuid, const QUuid &parentUuid, AxiomModel::ModelRoot *root);

        static QPoint nodeToControl(QPoint p) { return p * 2; }

        static QSize nodeToControl(QSize s) { return s * 2; }

        static QPoint controlToNodeFloor(QPoint p) {
            return {(int) floorf(p.x() / 2.f), (int) floorf(p.y() / 2.f)};
        }

        static QPoint controlToNodeCeil(QPoint p) {
            return {(int) ceilf(p.x() / 2.f), (int) ceilf(p.y() / 2.f)};
        }

        static QSize controlToNodeFloor(QSize s) {
            return {(int) floorf(s.width() / 2.f), (int) floorf(s.height() / 2.f)};
        }

        static QSize controlToNodeCeil(QSize s) {
            return {(int) ceilf(s.width() / 2.f), (int) ceilf(s.height() / 2.f)};
        }

        void serialize(QDataStream &stream) const override;

        ChildCollection &controls() { return _controls; }

        const ChildCollection &controls() const { return _controls; }

        GridSurface &grid() { return _grid; }

        const GridSurface &grid() const { return _grid; }

        void remove() override;

    private:
        ChildCollection _controls;
        GridSurface _grid;
    };

}
