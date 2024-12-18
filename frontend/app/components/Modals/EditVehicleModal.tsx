import { Dialog, DialogContent, DialogHeader, DialogTitle } from "~/components/ui/dialog";
import { Input } from "~/components/ui/input";
import { Button } from "~/components/ui/button";
import { Label } from "../ui/label";

interface EditVehicleModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (data: {
    id: string;
    brand: string;
    model: string;
    registration: string;
    registrationExpiry: string;
  }) => void;
  vehicle: {
    id: string;
    brand: string;
    model: string;
    registration: string;
    registrationExpiry: string;
  } | null;
}

export function EditVehicleModal({
  isOpen,
  onClose,
  onSubmit,
  vehicle,
}: EditVehicleModalProps) {
  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;
    const data = {
      id: vehicle!.id,
      brand: form.brand.value,
      model: form.model.value,
      registration: form.registration.value,
      registrationExpiry: form.registrationExpiry.value,
    };
    onSubmit(data);
  };

  if (!isOpen || !vehicle) return null;

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Vehicle</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit}>
          <div className="grid gap-4">
            <Label htmlFor="brand">Brand</Label>
            <Input
              id="brand"
              name="brand"
              defaultValue={vehicle.brand}
              required
            />
            <Label htmlFor="model">Model</Label>
            <Input
              id="model"
              name="model"
              defaultValue={vehicle.model}
              required
            />
            <Label htmlFor="registration">Registration</Label>
            <Input
              id="registration"
              name="registration"
              defaultValue={vehicle.registration}
              required
            />
            <Label htmlFor="registrationExpiry">Registration Expiry Date</Label>
            <Input
              id="registrationExpiry"
              name="registrationExpiry"
              type="date"
              defaultValue={vehicle.registrationExpiry}
              required
            />
          </div>
          <div className="flex justify-end space-x-4 mt-6">
            <Button variant="secondary" onClick={onClose}>
              Cancel
            </Button>
            <Button type="submit">Save Changes</Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
