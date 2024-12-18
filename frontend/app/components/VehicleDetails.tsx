import { Button } from "~/components/ui/button";

interface VehicleDetailsProps {
  vehicle: {
    id: string;
    brand: string;
    model: string;
    registration: string;
    registrationExpiry: string;
    createdAt: string;
    updatedAt: string;
  };
  onEdit: () => void;
  onDelete: () => void;
}

export function VehicleDetails({ vehicle, onEdit, onDelete }: VehicleDetailsProps) {
  return (
    <div className="bg-white rounded-lg shadow p-6">
      <div className="flex justify-between items-center mb-4">
        <h5 className="text-lg font-semibold text-gray-800">Vehicle Details</h5>
        <div className="space-x-2">
          <Button variant="secondary" onClick={onEdit}>
            Edit
          </Button>
          <Button variant="destructive" onClick={onDelete}>
            Delete
          </Button>
        </div>
      </div>
      <div className="space-y-4">
        {Object.entries(vehicle).map(([key, value]) => (
          <div key={key} className="flex justify-between">
            <span className="font-medium text-gray-600">{key.replace(/([A-Z])/g, " $1")}</span>
            <span className="text-gray-800">{value}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
