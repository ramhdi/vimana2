import { Link } from "@remix-run/react";
import { Button } from "~/components/ui/button";
import { Vehicle } from "~/types/vehicle";

interface VehicleCardProps extends Vehicle {
  onEdit: () => void;
  onDelete: () => void;
}

export function VehicleCard({
  id,
  brand,
  model,
  registration,
  registration_expiry_date,
  onEdit,
  onDelete,
}: VehicleCardProps) {
  return (
    <div className="bg-white shadow-md rounded-lg p-6 flex flex-col justify-between hover:shadow-lg transition-shadow">
      <Link to={`/vehicle/${id}`}>
        <h3 className="text-lg font-semibold text-gray-800">
          {brand} {model}
        </h3>
      </Link>
      <p className="text-sm text-gray-500 mt-2">
        Registration: <span className="font-medium">{registration}</span>
        <br />
        Expires: <span className="font-medium">{registration_expiry_date}</span>
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
