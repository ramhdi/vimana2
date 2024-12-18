import { Dialog, DialogContent, DialogHeader, DialogTitle } from "~/components/ui/dialog";
import { Input } from "~/components/ui/input";
import { Button } from "~/components/ui/button";
import { Label } from "../ui/label";

interface CreateVehicleModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (data: {
    brand: string;
    model: string;
    registration: string;
    registrationExpiry: string;
  }) => void;
}

export function CreateVehicleModal({ isOpen, onClose, onSubmit }: CreateVehicleModalProps) {
  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;
    const data = {
      brand: form.brand.value,
      model: form.model.value,
      registration: form.registration.value,
      registrationExpiry: form.registrationExpiry.value,
    };
    onSubmit(data);
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add New Vehicle</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit}>
          <div className="grid gap-4">
            <Label htmlFor="brand">Brand</Label>
            <Input id="brand" name="brand" required />
            <Label htmlFor="model">Model</Label>
            <Input id="model" name="model" required />
            <Label htmlFor="registration">Registration</Label>
            <Input id="registration" name="registration" required />
            <Label htmlFor="registrationExpiry">Registration Expiry Date</Label>
            <Input
              id="registrationExpiry"
              name="registrationExpiry"
              type="date"
              required
            />
          </div>
          <div className="flex justify-end space-x-4 mt-6">
            <Button variant="secondary" onClick={onClose}>
              Cancel
            </Button>
            <Button type="submit">Add</Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
