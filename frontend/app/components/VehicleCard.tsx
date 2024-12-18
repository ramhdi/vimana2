import { Button } from "~/components/ui/button";

interface VehicleCardProps {
  id: string;
  brand: string;
  model: string;
  registration: string;
  registrationExpiry: string;
  onEdit: () => void;
  onDelete: () => void;
}

export function VehicleCard({
  // id,
  brand,
  model,
  registration,
  registrationExpiry,
  onEdit,
  onDelete,
}: VehicleCardProps) {
  return (
    <div className="bg-white shadow-md rounded-lg p-6 flex flex-col justify-between hover:shadow-lg transition-shadow">
      <h3 className="text-lg font-semibold text-gray-800">
        {brand} {model}
      </h3>
      <p className="text-sm text-gray-500 mt-2">
        Registration: <span className="font-medium">{registration}</span>
        <br />
        Expires: <span className="font-medium">{registrationExpiry}</span>
      </p>
      <div className="flex items-center space-x-4 mt-4">
        <Button variant="secondary" onClick={onEdit}>
          Edit
        </Button>
        <Button variant="destructive" onClick={onDelete}>
          Delete
        </Button>
      </div>
    </div>
  );
}
