import { useState } from "react";
import { Navbar } from "~/components/Navbar";
import { VehicleCard } from "~/components/VehicleCard";
import { CreateVehicleModal } from "~/components/Modals/CreateVehicleModal";
import { EditVehicleModal } from "~/components/Modals/EditVehicleModal";
import { DeleteVehicleModal } from "~/components/Modals/DeleteVehicleModal";
import { Button } from "~/components/ui/button";

export default function Dashboard() {
  const [vehicles, setVehicles] = useState([
    { id: "1", brand: "Toyota", model: "Camry", registration: "ABC123", registrationExpiry: "2025-01-01" },
    { id: "2", brand: "Honda", model: "Civic", registration: "XYZ789", registrationExpiry: "2024-06-15" },
  ]);
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [isEditModalOpen, setIsEditModalOpen] = useState(false);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [selectedVehicle, setSelectedVehicle] = useState<any>(null);

  const handleAddVehicle = (vehicle: any) => {
    setVehicles([...vehicles, { ...vehicle, id: Date.now().toString() }]);
    setIsCreateModalOpen(false);
  };

  const handleEditVehicle = (updatedVehicle: any) => {
    setVehicles(vehicles.map((v) => (v.id === updatedVehicle.id ? updatedVehicle : v)));
    setIsEditModalOpen(false);
  };

  const handleDeleteVehicle = () => {
    setVehicles(vehicles.filter((v) => v.id !== selectedVehicle.id));
    setIsDeleteModalOpen(false);
  };

  return (
    <div>
      <Navbar />
      <div className="container mx-auto p-4">
        <div className="flex justify-between items-center">
          <h2 className="text-2xl font-bold text-gray-800">Your Vehicles</h2>
          <Button onClick={() => setIsCreateModalOpen(true)}>Add Vehicle</Button>
        </div>
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 mt-6">
          {vehicles.map((vehicle) => (
            <VehicleCard
              key={vehicle.id}
              {...vehicle}
              onEdit={() => {
                setSelectedVehicle(vehicle);
                setIsEditModalOpen(true);
              }}
              onDelete={() => {
                setSelectedVehicle(vehicle);
                setIsDeleteModalOpen(true);
              }}
            />
          ))}
        </div>
      </div>

      {/* Modals */}
      <CreateVehicleModal
        isOpen={isCreateModalOpen}
        onClose={() => setIsCreateModalOpen(false)}
        onSubmit={handleAddVehicle}
      />
      <EditVehicleModal
        isOpen={isEditModalOpen}
        onClose={() => setIsEditModalOpen(false)}
        onSubmit={handleEditVehicle}
        vehicle={selectedVehicle}
      />
      <DeleteVehicleModal
        isOpen={isDeleteModalOpen}
        onClose={() => setIsDeleteModalOpen(false)}
        onConfirm={handleDeleteVehicle}
        vehicle={selectedVehicle}
      />
    </div>
  );
}
