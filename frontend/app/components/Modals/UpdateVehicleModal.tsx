import { Dialog, DialogContent, DialogHeader, DialogTitle } from "~/components/ui/dialog";
import { Input } from "~/components/ui/input";
import { Button } from "~/components/ui/button";

export function UpdateVehicleModal({ isOpen, onClose, vehicle, onSubmit }: any) {
  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Vehicle</DialogTitle>
        </DialogHeader>
        <form onSubmit={(e) => {
          e.preventDefault();
          onSubmit({
            brand: e.target.brand.value,
            model: e.target.model.value,
            registration: e.target.registration.value,
            registrationExpiryDate: e.target.registrationExpiryDate.value,
          });
        }}>
          <Input id="brand" defaultValue={vehicle.brand} required />
          <Input id="model" defaultValue={vehicle.model} required />
          <Input id="registration" defaultValue={vehicle.registration} required />
          <Input id="registrationExpiryDate" defaultValue={vehicle.registrationExpiryDate} required type="date" />
          <div className="flex justify-end mt-4 space-x-2">
            <Button variant="secondary" onClick={onClose}>Cancel</Button>
            <Button type="submit">Save</Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
