package com.example.examplemod.entity.client;

import com.example.examplemod.MyMod;

import net.minecraft.client.model.geom.ModelLayerLocation;
import net.minecraft.resources.ResourceLocation;

public class ModModelLayers {
    public static final ModelLayerLocation RHINO_LAYER = new ModelLayerLocation(
            ResourceLocation.fromNamespaceAndPath(MyMod.MODID, "rhino_layer"), "main");
}
