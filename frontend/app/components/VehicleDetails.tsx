import { Button } from "~/components/ui/button";
import { Vehicle } from "~/types/vehicle";

interface VehicleDetailsProps {
  vehicle: Vehicle;
  onEdit: () => void;
  onDelete: () => void;
}

export function VehicleDetails({ vehicle, onEdit, onDelete }: VehicleDetailsProps) {
  const humanReadableFields: { key: keyof Vehicle; label: string }[] = [
    { key: "brand", label: "Brand" },
    { key: "model", label: "Model" },
    { key: "registration", label: "Registration" },
    { key: "registration_expiry_date", label: "Registration Expiry Date" },
    { key: "created_at", label: "Created At" },
    { key: "updated_at", label: "Updated At" },
  ];

  const formatDate = (date: string, locale: string = "en-US") => {
    const options: Intl.DateTimeFormatOptions = {
      year: "numeric",
      month: "short",
      day: "numeric",
    };
    return new Intl.DateTimeFormat(locale, options).format(new Date(date));
  };


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
        {humanReadableFields.map(({ key, label }) => (
          <div key={key} className="flex justify-between border-b pb-2">
            <span className="font-medium text-gray-600">{label}</span>
            <span className="text-gray-800">
              {["registration_expiry_date", "created_at", "updated_at"].includes(key) ? formatDate(vehicle[key] as string) : vehicle[key]}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}
