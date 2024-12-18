import { Dialog, DialogContent, DialogHeader, DialogTitle } from "~/components/ui/dialog";
import { Input } from "~/components/ui/input";
import { Button } from "~/components/ui/button";
import { Label } from "../ui/label";
import { Vehicle } from "~/types/vehicle";

interface VehicleModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: ((data: Omit<Vehicle, "id">) => void) | ((updatedVehicle: Vehicle) => void);
  vehicle?: Vehicle | null;
  mode: "create" | "edit";
}

export function VehicleModal({ isOpen, onClose, onSubmit, vehicle, mode }: VehicleModalProps) {
  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;
    const data = {
      id: vehicle?.id, // Only include ID in edit mode
      brand: form.brand.value,
      model: form.model.value,
      registration: form.registration.value,
      registration_expiry_date: form.registration_expiry_date.value,
    };
    onSubmit(data as Vehicle);
  };

  const isEditMode = mode === "edit";

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{isEditMode ? "Edit Vehicle" : "Add New Vehicle"}</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit}>
          <div className="grid gap-4">
            <Label htmlFor="brand">Brand</Label>
            <Input id="brand" name="brand" defaultValue={vehicle?.brand || ""} required />
            <Label htmlFor="model">Model</Label>
            <Input id="model" name="model" defaultValue={vehicle?.model || ""} required />
            <Label htmlFor="registration">Registration</Label>
            <Input id="registration" name="registration" defaultValue={vehicle?.registration || ""} required />
            <Label htmlFor="registration_expiry_date">Registration Expiry Date</Label>
            <Input
              id="registration_expiry_date"
              name="registration_expiry_date"
              type="date"
              defaultValue={vehicle?.registration_expiry_date || ""}
              required
            />
          </div>
          <div className="flex justify-end space-x-4 mt-6">
            <Button variant="secondary" onClick={onClose}>
              Cancel
            </Button>
            <Button type="submit">{isEditMode ? "Save Changes" : "Add"}</Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
