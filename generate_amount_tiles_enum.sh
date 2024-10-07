#!/bin/bash

# Function to calculate the amount of tiles based on the given radius
radius_to_amount_tiles() {
  local radius=$1
  echo $((3 * radius * radius + 3 * radius + 1))
}

# Print the start of the enum definition
echo "pub enum RadiusAndAmountTiles {"

# Loop through 0 to 255 to generate each enum variant
for radius in {0..255}; do
  # Calculate the amount of tiles for the current radius
  amount=$(radius_to_amount_tiles $radius)
  # Print the enum variant with the calculated amount
  echo "    Radius${radius}Amount${amount} = ${radius},"
done

# Print the end of the enum definition
echo "}"

# Start the impl block
echo "impl RadiusAndAmountTiles {"

# Generate the radius() function
echo "    pub const fn radius(&self) -> u32 {"
echo "        match self {"
for radius in {0..255}; do
  amount=$(radius_to_amount_tiles $radius)
  echo "            RadiusAndAmountTiles::Radius${radius}Amount${amount} => ${radius},"
done
echo "        }"
echo "    }"

# Generate the amount_tiles() function
echo ""
echo "    pub const fn amount_tiles(&self) -> usize {"
echo "        match self {"
for radius in {0..255}; do
  amount=$(radius_to_amount_tiles $radius)
  echo "            RadiusAndAmountTiles::Radius${radius}Amount${amount} => ${amount},"
done
echo "        }"
echo "    }"

# Generate the from_radius() function
echo ""
echo "    pub const fn from_radius(radius: u32) -> Option<Self> {"
echo "        match radius {"
for radius in {0..255}; do
  amount=$(radius_to_amount_tiles $radius)
  echo "            ${radius} => Some(RadiusAndAmountTiles::Radius${radius}Amount${amount}),"
done
echo "            _ => None,"
echo "        }"
echo "    }"

# Generate the from_amount_tiles() function
echo ""
echo "    pub const fn from_amount_tiles(amount: usize) -> Option<Self> {"
echo "        match amount {"
for radius in {0..255}; do
  amount=$(radius_to_amount_tiles $radius)
  echo "            ${amount} => Some(RadiusAndAmountTiles::Radius${radius}Amount${amount}),"
done
echo "            _ => None,"
echo "        }"
echo "    }"

# End the impl block
echo "}"
