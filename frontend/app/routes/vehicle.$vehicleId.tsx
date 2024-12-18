import { useState } from "react";
import { useParams } from "@remix-run/react";
import { Navbar } from "~/components/Navbar";
import { VehicleDetails } from "~/components/VehicleDetails";
import { AnalysisSection } from "~/components/AnalysisSection";
import { EditVehicleModal } from "~/components/Modals/EditVehicleModal";
import { DeleteVehicleModal } from "~/components/Modals/DeleteVehicleModal";

const vehicles = [
  {
    id: "1", brand: "Toyota", model: "Camry", registration: "ABC123", registrationExpiry: "2025-01-01", createdAt: "2022-01-01",
    updatedAt: "2023-01-01"
  },
  {
    id: "2", brand: "Honda", model: "Civic", registration: "XYZ789", registrationExpiry: "2024-06-15", createdAt: "2022-01-01",
    updatedAt: "2023-01-01",
  },
];

export default function VehiclePage() {
  const { vehicleId } = useParams();
  if (!vehicleId) {
    throw new Response("Vehicle not found", { status: 404 });
  }

  const vehicle = vehicles.find((v) => v.id === vehicleId);
  if (!vehicle) {
    throw new Response("Vehicle not found", { status: 404 });
  }

  const [isUpdateModalOpen, setIsUpdateModalOpen] = useState(false);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);

  return (
    <>
      <Navbar />
      <div className="container mx-auto p-4 space-y-6">
        <div className="flex flex-col lg:flex-row gap-6">
          {/* Vehicle Details */}
          <div className="lg:w-1/3">
            <VehicleDetails
              vehicle={vehicle}
              onEdit={() => setIsUpdateModalOpen(true)}
              onDelete={() => setIsDeleteModalOpen(true)}
            />
          </div>

          {/* Usage & Performance Analysis */}
          <div className="lg:w-2/3">
            <AnalysisSection />
          </div>
        </div>
      </div>

      {/* Modals */}
      <EditVehicleModal
        isOpen={isUpdateModalOpen}
        onClose={() => setIsUpdateModalOpen(false)}
        vehicle={vehicle}
        onSubmit={(data) => console.log("Update data:", data)} // Dummy handler
      />
      <DeleteVehicleModal
        isOpen={isDeleteModalOpen}
        onClose={() => setIsDeleteModalOpen(false)}
        vehicle={vehicle}
        onConfirm={() => console.log("Vehicle deleted")} // Dummy handler
      />
    </>
  );
}
