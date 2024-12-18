import { Dialog, DialogContent, DialogHeader, DialogTitle } from "~/components/ui/dialog";
import { Button } from "~/components/ui/button";

interface DeleteVehicleModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => void;
  vehicle: {
    id: string;
    brand: string;
    model: string;
  } | null;
}

export function DeleteVehicleModal({
  isOpen,
  onClose,
  onConfirm,
  vehicle,
}: DeleteVehicleModalProps) {
  if (!isOpen || !vehicle) return null;

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Confirm Deletion</DialogTitle>
        </DialogHeader>
        <div className="mt-4">
          <p className="text-sm text-gray-600">
            Are you sure you want to delete the vehicle{" "}
            <span className="font-bold">
              {vehicle.brand} {vehicle.model}
            </span>
            ? This action cannot be undone.
          </p>
        </div>
        <div className="flex justify-end space-x-4 mt-6">
          <Button variant="secondary" onClick={onClose}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={onConfirm}>
            Delete
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
